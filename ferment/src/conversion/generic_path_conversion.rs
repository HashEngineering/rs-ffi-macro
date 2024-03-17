use std::clone::Clone;
use std::fmt::{Debug, Display, Formatter};
use proc_macro2::Ident;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::{parse_quote, Path, PathSegment, Type};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma};
use crate::composer::{ConstructorPresentableContext, Depunctuated};
use crate::conversion::type_composition_conversion::TypeCompositionConversion;
use crate::context::ScopeContext;
use crate::conversion::{FieldTypeConversion, PathConversion};
use crate::ext::{Accessory, FFIResolver, Mangle};
use crate::helper::{path_arguments_to_path_conversions};
use crate::interface::{create_struct, ffi_to_conversion};
use crate::naming::{DictionaryFieldName, Name};
use crate::presentation::ffi_object_presentation::FFIObjectPresentation;
use crate::presentation::{InterfacePresentation, DropInterfacePresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation};
use crate::presentation::context::{BindingPresentableContext, IteratorPresentationContext, OwnedItemPresentableContext};
use crate::presentation::destroy_presentation::DestroyPresentation;
use crate::wrapped::Wrapped;

// pub const BODY_ARG_PRESENTER: OwnedFieldTypeComposerRef =
//     |field| OwnedItemPresentableContext::DefaultField(field.clone());

pub struct GenericArgPresentation {
    pub ty: Type,
    pub destructor: TokenStream2,
    pub from_conversion: TokenStream2,
    pub to_conversion: TokenStream2,
}

impl Debug for GenericArgPresentation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("GenericArgPresentation({})", self.ty.to_token_stream()))
    }
}
impl Display for GenericArgPresentation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl GenericArgPresentation {
    pub fn new(ty: Type, destructor: TokenStream2, from_conversion: TokenStream2, to_conversion: TokenStream2) -> Self {
        Self { ty, destructor, from_conversion, to_conversion }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum GenericPathConversion {
    Map(Path),
    Vec(Path),
    Result(Path),
    Box(Path),
    AnyOther(Path),
    Tuple(Punctuated<Path, Comma>)
}
impl ToTokens for GenericPathConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            GenericPathConversion::Map(path) |
            GenericPathConversion::Vec(path) |
            GenericPathConversion::Result(path) |
            GenericPathConversion::Box(path) |
            GenericPathConversion::AnyOther(path) => path.to_tokens(tokens),
            GenericPathConversion::Tuple(conversions) => conversions.to_tokens(tokens)
        }
    }
}


fn single_generic_ffi_path(path: &Path) -> Type {
    let mut cloned_segments = path.segments.clone();
    let last_segment = cloned_segments.iter_mut().last().unwrap();
    let last_ident = &last_segment.ident;
    match last_ident.to_string().as_str() {
        // simple primitive type
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "f64" | "u64" | "i128" | "u128" |
        "isize" | "usize" | "bool" => parse_quote!(#last_ident),
        // complex special type
        "str" | "String" => parse_quote!(std::os::raw::c_char),
        "Vec" | "BTreeMap" | "HashMap" => {
            let ffi_name = path.to_mangled_ident_default();
            parse_quote!(crate::fermented::generics::#ffi_name)
        },
        "Result" if cloned_segments.len() == 1 => {
            let ffi_name = path.to_mangled_ident_default();
            parse_quote!(crate::fermented::generics::#ffi_name)

        },
        _ => {
            let new_segments = cloned_segments
                .into_iter()
                .map(|segment| quote_spanned! { segment.span() => #segment })
                .collect::<Vec<_>>();
            parse_quote!(#(#new_segments)::*)
        }
    }
}
impl GenericPathConversion {
    pub fn to_ffi_path(&self) -> Type {
        match self {
            GenericPathConversion::Map(path) |
            GenericPathConversion::Vec(path) |
            GenericPathConversion::Result(path) |
            GenericPathConversion::Box(path) |
            GenericPathConversion::AnyOther(path) => single_generic_ffi_path(path),
            GenericPathConversion::Tuple(paths) => {
                match paths.len() {
                    0 => single_generic_ffi_path(paths.first().unwrap()),
                    _ => {
                        let ffi_name = format_ident!("{}_Tuple", paths.iter().map(|p| p.to_mangled_ident_default().to_string()).collect::<Vec<_>>().join("_"));
                        parse_quote!(crate::fermented::generics::#ffi_name)
                    }
                }
            }
        }
    }
}
impl GenericPathConversion {
    pub fn expand(&self, full_type: &TypeCompositionConversion, source: &ScopeContext) -> TokenStream2 {
        println!("GenericPathConversion::expand: {}", full_type.to_token_stream());
        let ffi_type = full_type.ty();
        let ffi_name = ffi_type.to_mangled_ident_default();
        let ffi_as_type: Type = parse_quote!(#ffi_name);

        match self {
            GenericPathConversion::Result(path) => {
                let PathSegment { arguments, .. } = path.segments.last().unwrap();
                let path_conversions = path_arguments_to_path_conversions(arguments);
                let arg_0_name = Name::Dictionary(DictionaryFieldName::Ok);
                let arg_1_name = Name::Dictionary(DictionaryFieldName::Error);

                let (arg_0_presentation, arg_1_presentation) = match &path_conversions[..] {
                    [PathConversion::Primitive(ok), PathConversion::Primitive(error)] => {
                        let arg_0_ffi_type = parse_quote!(#ok);
                        let arg_1_ffi_type = parse_quote!(#error);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_primitive_enum(),
                                ffi_to_primitive_enum()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_primitive_enum(),
                                ffi_to_primitive_enum()),
                        )
                    },
                    [PathConversion::Primitive(ok), PathConversion::Complex(error)] => {
                        let arg_0_ffi_type = parse_quote!(#ok);
                        let arg_1_ffi_type = error.ffi_path_converted_or_same(source);
                        let arg_1_ffi_type: Type = parse_quote!(#arg_1_ffi_type);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_primitive_enum(),
                                ffi_to_primitive_enum()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                        )
                    },
                    [PathConversion::Primitive(ok), PathConversion::Generic(generic_error)] => {
                        let arg_0_ffi_type = parse_quote!(#ok);
                        let arg_1_ffi_type = generic_error.to_ffi_path();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_primitive_enum(),
                                ffi_to_primitive_enum()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Primitive(error)] => {
                        let arg_0_ffi_type = ok.ffi_path_converted_or_same(source);
                        let arg_0_ffi_type: Type = parse_quote!(#arg_0_ffi_type);
                        let arg_1_ffi_type = parse_quote!(#error);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_primitive_enum(),
                                ffi_to_primitive_enum()),
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Complex(error)] => {
                        let arg_0_ffi_type = ok.ffi_path_converted_or_same(source);
                        let arg_1_ffi_type = error.ffi_path_converted_or_same(source);
                        let arg_0_ffi_type = parse_quote!(#arg_0_ffi_type);
                        let arg_1_ffi_type = parse_quote!(#arg_1_ffi_type);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Generic(generic_error)] => {
                        let arg_0_ffi_type = ok.ffi_path_converted_or_same(source);
                        let arg_0_ffi_type = parse_quote!(#arg_0_ffi_type);
                        let arg_1_ffi_type = generic_error.to_ffi_path();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Primitive(error)] => {
                        let arg_0_ffi_type = generic_ok.to_ffi_path();
                        let arg_1_ffi_type = parse_quote!(#error);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_primitive_enum(),
                                ffi_to_primitive_enum()),
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Complex(error)] => {
                        let arg_0_ffi_type = generic_ok.to_ffi_path();
                        let arg_1_ffi_type = error.ffi_path_converted_or_same(source);
                        let arg_1_ffi_type: Type = parse_quote!(#arg_1_ffi_type);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Generic(generic_error)] => {
                        let arg_0_ffi_type = generic_ok.to_ffi_path();
                        let arg_1_ffi_type = generic_error.to_ffi_path();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                        )
                    },
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                let target_type: Type = parse_quote!(#path);
                let GenericArgPresentation { ty: ok_type, from_conversion: from_ok_conversion, to_conversion: to_ok_conversion, destructor: ok_destructor } = arg_0_presentation;
                let GenericArgPresentation { ty: error_type, from_conversion: from_error_conversion, to_conversion: to_error_conversion, destructor: error_destructor } = arg_1_presentation;
                compose_generic_presentation(
                    ffi_name,
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(arg_0_name, ok_type.joined_mut()),
                        FieldTypeConversion::Named(arg_1_name, error_type.joined_mut()),
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Result(quote!(#from_ok_conversion), quote!(#from_error_conversion)),
                                ToConversionPresentation::Result(quote!(#to_ok_conversion), quote!(#to_error_conversion)),
                                DestroyPresentation::Default,
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter([ok_destructor, error_destructor]),
                    source
                )
            },
            GenericPathConversion::Map(path) => {
                let PathSegment { arguments, .. } = path.segments.last().unwrap();
                let path_conversions = path_arguments_to_path_conversions(arguments);
                let arg_0_name = Name::Dictionary(DictionaryFieldName::Keys);
                let arg_1_name = Name::Dictionary(DictionaryFieldName::Values);
                let count_name = Name::Dictionary(DictionaryFieldName::Count);
                let (arg_0_presentation, arg_1_presentation) = match &path_conversions[..] {
                    [PathConversion::Primitive(arg_0_target_path), PathConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_0_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_1_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_values()),
                        )
                    }
                    [PathConversion::Primitive(arg_0_target_path), PathConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = arg_1_target_path.ffi_path_converted_or_same(source);
                        let arg_1_ffi_type: Type = parse_quote!(#arg_1_ffi_type);
                        let arg_1_ffi_type = arg_1_ffi_type.joined_mut();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_0_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values())
                        )
                    }
                    [PathConversion::Primitive(arg_0_target_path), PathConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_ffi_path().joined_mut();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_0_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    [PathConversion::Complex(arg_0_target_path), PathConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_target_path.ffi_path_converted_or_same(source);
                        let arg_0_ffi_type: Type = parse_quote!(#arg_0_ffi_type);
                        let arg_0_ffi_type = arg_0_ffi_type.joined_mut();
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_values()),
                        )
                    }
                    [PathConversion::Complex(arg_0_target_path), PathConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_target_path.ffi_path_converted_or_same(source);
                        let arg_1_ffi_type = arg_1_target_path.ffi_path_converted_or_same(source);
                        let arg_0_ffi_type: Type = parse_quote!(#arg_0_ffi_type);
                        let arg_1_ffi_type: Type = parse_quote!(#arg_1_ffi_type);
                        let arg_0_ffi_type = arg_0_ffi_type.joined_mut();
                        let arg_1_ffi_type = arg_1_ffi_type.joined_mut();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    [PathConversion::Complex(arg_0_target_path), PathConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_target_path.ffi_path_converted_or_same(source);
                        let arg_0_ffi_type: Type = parse_quote!(#arg_0_ffi_type);
                        let arg_0_ffi_type = arg_0_ffi_type.joined_mut();
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_ffi_path().joined_mut();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    [PathConversion::Generic(arg_0_generic_path_conversion), PathConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_ffi_path().joined_mut();
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_1_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_values()),
                        )
                    }
                    [PathConversion::Generic(arg_0_generic_path_conversion), PathConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_ffi_path().joined_mut();
                        let arg_1_ffi_type = arg_1_target_path.ffi_path_converted_or_same(source);
                        let arg_1_ffi_type: Type = parse_quote!(#arg_1_ffi_type);
                        let arg_1_ffi_type = arg_1_ffi_type.joined_mut();

                        let arg_1_ffi_type = parse_quote!(#arg_1_ffi_type);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    [PathConversion::Generic(arg_0_generic_path_conversion), PathConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_ffi_path().joined_mut();
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_ffi_path().joined_mut();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                let target_type: Type = parse_quote!(#path);
                let GenericArgPresentation { ty: key, from_conversion: from_key_conversion, to_conversion: to_key_conversion, destructor: key_destructor } = arg_0_presentation;
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = arg_1_presentation;
                compose_generic_presentation(
                    ffi_name,
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(count_name, parse_quote!(usize)),
                        FieldTypeConversion::Named(arg_0_name,key.joined_mut()),
                        FieldTypeConversion::Named(arg_1_name, value.joined_mut())
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Map(quote!(#from_key_conversion), quote!(#from_value_conversion)),
                                ToConversionPresentation::Map(quote!(#to_key_conversion), quote!(#to_value_conversion)),
                                DestroyPresentation::Default,
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter([key_destructor, value_destructor]),
                    source
                )
            },
            GenericPathConversion::Vec(path) => {
                let PathSegment { arguments, .. } = path.segments.last().unwrap();
                let path_conversions = path_arguments_to_path_conversions(arguments);
                let arg_0_name = Name::Dictionary(DictionaryFieldName::Values);
                let count_name = Name::Dictionary(DictionaryFieldName::Count);
                let arg_0_presentation = match &path_conversions[..] {
                    [PathConversion::Primitive(arg_0_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_primitive_vec(quote!(self.#arg_0_name)),
                            DictionaryFieldName::FromPrimitiveVec(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryFieldName::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::boxed_vec(obj) })).to_token_stream())
                    }
                    [PathConversion::Complex(arg_0_target_path)] => {
                        let arg_0_ffi_type = arg_0_target_path.ffi_path_converted_or_same(source);
                        let arg_0_ffi_type: Type = parse_quote!(#arg_0_ffi_type);
                        let arg_0_ffi_type = arg_0_ffi_type.joined_mut();

                        let arg_0_ffi_type = parse_quote!(#arg_0_ffi_type);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                            DictionaryFieldName::FromComplexVec(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryFieldName::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) })).to_token_stream())
                    }
                    [PathConversion::Generic(arg_0_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_ffi_path().joined_mut();
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                            DictionaryFieldName::FromComplexVec(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryFieldName::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) })).to_token_stream())
                    }
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                let target_type: Type = parse_quote!(#path);
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = arg_0_presentation;

                compose_generic_presentation(
                    ffi_name,
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(count_name, parse_quote!(usize)),
                        FieldTypeConversion::Named(arg_0_name, value.joined_mut())
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Just(quote!(ferment_interfaces::FFIVecConversion::decode(&*ffi))),
                                ToConversionPresentation::Struct(quote!(ferment_interfaces::FFIVecConversion::encode(obj))),
                                DestroyPresentation::Default,
                                None
                            )
                        },
                        InterfacePresentation::VecConversion {
                            types: (ffi_as_type.clone(), target_type.clone()),
                            decode: from_value_conversion,
                            encode: to_value_conversion,
                        }
                    ]),
                    Depunctuated::from_iter([value_destructor]),
                    source
                )
            },
            GenericPathConversion::Tuple(tuple) => {
                // match tuple.len() {
                //     0 => single_generic_ffi_path(tuple.first().unwrap()),
                //     _ => {
                //         // let ffi_name = format_ident!("{}_Tuple", paths.iter().map(|p| p.to_mangled_ident_default().to_string()).collect::<Vec<_>>().join("_"));
                //         // parse_quote!(crate::fermented::generics::#ffi_name)
                //     }
                // }
                //
                println!("GenericPathConversion::Tuple: {}", tuple.to_token_stream());
                let tuple_items = tuple.iter().enumerate().map(|(index, path)| {
                    let root_path = match PathConversion::from(path) {
                        PathConversion::Primitive(root_path) => parse_quote!(#root_path),
                        PathConversion::Complex(root_path) => parse_quote!(#root_path),
                        PathConversion::Generic(root_path) => root_path.to_ffi_path()
                    };
                    let PathSegment { arguments, .. } = path.segments.last().unwrap();
                    let path_conversions = path_arguments_to_path_conversions(arguments);
                    let generic_args = path_conversions.iter().enumerate().map(|(index, arg_path_conversion)| {
                        match arg_path_conversion {
                            PathConversion::Primitive(arg_path) => {
                                GenericArgPresentation::new(
                                    parse_quote!(#arg_path),
                                    quote!(),
                                    ffi_from_primitive(),
                                    ffi_to_primitive())
                            },
                            PathConversion::Complex(arg_path) => {
                                GenericArgPresentation::new(
                                    parse_quote!(#arg_path),
                                    quote!(),
                                    ffi_from_primitive(),
                                    ffi_to_primitive())
                            },
                            PathConversion::Generic(arg_path) => {
                                GenericArgPresentation::new(
                                    parse_quote!(#arg_path),
                                    quote!(),
                                    ffi_from_primitive(),
                                    ffi_to_primitive())
                            },
                        }
                    }).collect::<Depunctuated<GenericArgPresentation>>();
                    (root_path, generic_args)
                }).collect::<Depunctuated<(Type, Depunctuated<GenericArgPresentation>)>>();
                compose_generic_presentation(
                    ffi_name,
                    Depunctuated::from_iter(
                        tuple_items.iter()
                            .enumerate()
                            .map(|(index, (root_path, arg))| FieldTypeConversion::Unnamed(Name::UnnamedArg(index), parse_quote!(#root_path)))),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            types: (ffi_as_type.clone(), parse_quote!((#tuple))),
                            conversions: (
                                FromConversionPresentation::Tuple(tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.from_conversion.clone())).collect()),
                                ToConversionPresentation::Tuple(tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.to_conversion.clone())).collect()),
                                DestroyPresentation::Default,
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter(tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.destructor.clone()))),
                    source
                )
            },
            GenericPathConversion::Box(_path) | GenericPathConversion::AnyOther(_path) => FFIObjectPresentation::Empty
        }.to_token_stream()
    }
}

fn compose_generic_presentation(
    ffi_name: Ident,
    field_conversions: Depunctuated<FieldTypeConversion>,
    interface_presentations: Depunctuated<InterfacePresentation>,
    drop_body: Depunctuated<TokenStream2>,
    source: &ScopeContext) -> FFIObjectPresentation {
    let ffi_as_path: Path = parse_quote!(#ffi_name);
    let ffi_as_type: Type = parse_quote!(#ffi_name);
    let fields = Punctuated::<_, Comma>::from_iter(field_conversions.iter().map(|field| OwnedItemPresentableContext::Named(field.clone(), true)));
    let body = Wrapped::<_, Brace>::new(fields.present(&source));
    let object_presentation = create_struct(&ffi_as_path, body.to_token_stream());
    let drop_presentation = DropInterfacePresentation::Full { ty: ffi_as_type.clone(), body: drop_body.to_token_stream() };
    let bindings = compose_bindings(&ffi_as_type, field_conversions).present(&source);
    FFIObjectPresentation::Generic { object_presentation, interface_presentations, drop_presentation, bindings }
}

fn compose_bindings(ffi_type: &Type, conversions: Depunctuated<FieldTypeConversion>) -> Depunctuated<BindingPresentableContext> {
    Depunctuated::from_iter([
        BindingPresentableContext::Constructor(
            ConstructorPresentableContext::Default(ffi_type.clone()),
            conversions.iter().map(|field| OwnedItemPresentableContext::Named(field.clone(), false)).collect(),
            IteratorPresentationContext::Curly(conversions.iter().map(|field| OwnedItemPresentableContext::DefaultField(field.clone())).collect())),
        BindingPresentableContext::Destructor(ffi_type.clone())
    ])
}


fn ffi_from_primitive() -> TokenStream2 {
    quote!(|o| o)
}

fn ffi_from_complex() -> TokenStream2 {
    quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o))
}
fn ffi_from_primitive_enum() -> TokenStream2 {
    quote!(|o| *o)
}

fn ffi_to_complex() -> TokenStream2 {
    ffi_to_conversion(quote!(o))
}

fn ffi_to_primitive() -> TokenStream2 {
    quote!(o)
}

fn ffi_to_primitive_enum() -> TokenStream2 {
    quote!(o as *mut _)
}

fn ffi_to_primitive_keys() -> TokenStream2 {
    quote!(ferment_interfaces::to_primitive_vec(obj.keys().cloned()))
}
fn ffi_to_complex_keys() -> TokenStream2 {
    quote!(ferment_interfaces::to_complex_vec(obj.keys().cloned()))
}
fn ffi_to_primitive_values() -> TokenStream2 {
    quote!(ferment_interfaces::to_primitive_vec(obj.values().cloned()))
}
fn ffi_to_complex_values() -> TokenStream2 {
    quote!(ferment_interfaces::to_complex_vec(obj.values().cloned()))
}
fn ffi_to_primitive_vec() -> TokenStream2 {
    quote!(ferment_interfaces::boxed_vec(obj))
}
fn ffi_to_complex_vec() -> TokenStream2 {
    quote!(ferment_interfaces::to_complex_vec(obj.into_iter()))
}

fn ffi_destroy_primitive_vec(field_name: TokenStream2) -> TokenStream2 {
    let count_var = DictionaryFieldName::Count;
    quote!(ferment_interfaces::unbox_vec_ptr(#field_name, self.#count_var);)
}
fn ffi_destroy_complex_vec(field_name: TokenStream2) -> TokenStream2 {
    let count_var = DictionaryFieldName::Count;
    quote!(ferment_interfaces::unbox_any_vec_ptr(#field_name, self.#count_var);)
}
fn ffi_destroy_option(field_name: TokenStream2) -> TokenStream2 {
    quote!(if !#field_name.is_null() { ferment_interfaces::unbox_any(#field_name) };)
}

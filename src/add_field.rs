use proc_macro::TokenStream;
use quote::quote;



pub trait AddField {
    fn add_field(&mut self, field : syn::Field);
}

impl AddField for syn::ItemStruct {
    fn add_field(&mut self, field : syn::Field) {
        if let syn::Fields::Named(ref mut fields) = self.fields {
            fields.named.push(field);
        }
    }
}

pub struct Args {
    pub field : syn::Field,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vars = syn::punctuated::Punctuated::<syn::LitStr, syn::Token![,]>::parse_terminated(input)?;
        let field = if let [vis, name, ty] = vars.into_iter().collect::<Vec<syn::LitStr>>().as_slice() {
            let vis : syn::Visibility = vis.parse()?;
            let name = syn::Ident::new(name.value().as_str(), name.span());
            let ty : syn::Type = ty.parse()?;
            syn::parse::Parser::parse2(syn::Field::parse_named, quote!{ #vis #name : #ty})?
        } else {
            return Err(syn::Error::new(input.span(), "The macro expect a field as such: #[add_field(\"vis\",\"name\",\"type\")]."));
        };
        Ok(Self {
            field,
        })
    }
}

use std::collections::HashMap;

use nal_symbol::Symbol;

use crate::primitive::{self, Primitive};

use self::Ty::*;

#[derive(Debug, Clone)]
pub enum Ty {
    Record(HashMap<Symbol, Ty>),
    Enum(HashMap<Symbol, Ty>),
    Tuple(Vec<Ty>),
    Primitive(Primitive),
}

impl Ty {
    pub fn is_subtype_of(&self, that: &Ty) -> bool {
        match (self, that) {
            (Record(this), Record(that)) => {
                that.iter().all(|(name, field)| {
                    this.get(name).map_or(false, |this| this.is_subtype_of(field))
                })
            }
            (Enum(this), Enum(that)) => {
                this.iter().all(|(name, variant)| {
                    that.get(name).map_or(false, |that| that.is_subtype_of(variant))
                })
            }
            (Enum(this), _) => {
                this.values().all(|variant| {
                    variant.is_subtype_of(that)
                })
            }
            (_, Enum(_)) => false,
            (Tuple(this), Tuple(that)) => {
                this.len() == that.len() &&
                    this.iter().zip(that).all(|(this, that)| this.is_subtype_of(that))
            }
            (Tuple(this), _) => {
                this.len() == 1 && this[0].is_subtype_of(that)
            }
            (_, Tuple(that)) => {
                that.len() == 1 && self.is_subtype_of(&that[0])
            }
            (Primitive(this), Primitive(that)) => this == that,
            (_, Primitive(_)) => false, // primitives can't be other type's supertype
            (Primitive(this), _) => primitive::wrap(*this).is_subtype_of(that),
        }
    }
}

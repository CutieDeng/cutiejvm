
pub mod reference_kind_literals {
    use crate::defu8;

    defu8!(GET_FIELD, 1); 
    defu8!(GET_STATIC, 2); 
    defu8!(PUT_FIELD, 3); 
    defu8!(PUT_STATIC, 4); 

    defu8!(INVOKE_VIRTUAL, 5); 
    defu8!(INVOKE_STATIC, 6); 
    defu8!(INVOKE_SPECIAL, 7); 
    defu8!(NEW_INVOKE_SPECIAL, 8); 

    defu8!(INVOKE_INTERFACE, 9); 

}

/// RefTypeEnum is used to describe the type of the `reference_index` in the `CONSTANT_MethodHandle_info` structure.
/// 
/// The `reference_index` is used to point to the `CONSTANT_Fieldref_info` or `CONSTANT_Methodref_info` or `CONSTANT_InterfaceMethodref_info` structure.
/// 
/// ### Special case 
/// If the method is `<init>` or `<clinit>`, the `reference_kind` must be `NEW_INVOKE_SPECIAL`.
/// 
/// 
pub enum RefType {
    /// The `get_field` instruction. 
    /// 
    /// The `reference_index` must be the type of `CONSTANT_Fieldref_info`.
    GetField, 

    /// The `get_static` instruction.
    /// 
    /// The `reference_index` must be the type of `CONSTANT_Fieldref_info`.
    GetStatic, 

    /// The `put_field` instruction.
    /// 
    /// The `reference_index` must be the type of `CONSTANT_Fieldref_info`.
    PutField, 

    /// The `put_static` instruction.
    /// 
    /// The `reference_index` must be the type of `CONSTANT_Fieldref_info`.
    PutStatic, 

    /// The `invoke_virtual` instruction.
    /// 
    /// The `reference_index` must be the type of `CONSTANT_Methodref_info`.
    InvokeVirtual, 

    /// The `invoke_virtual` instruction.
    /// 
    /// The `reference_index` must be the type of `CONSTANT_Methodref_info`.
    InvokeStatic, 

    /// The `invoke_special` instruction.
    /// 
    /// The `reference_index` must be the type of `CONSTANT_Methodref_info`.
    InvokeSpecial, 

    /// The `new_invoke_special` instruction.
    /// 
    /// The `reference_index` must be the type of `CONSTANT_Methodref_info`.
    NewInvokeSpecial, 

    /// The `invoke_interface` instruction.
    /// 
    /// The `reference_index` must be the type of `CONSTANT_InterfaceMethodref_info`.
    InvokeInterface, 
}

impl RefType {
    /// According to the `CONSTANT_MethodHandle_info` structure description, transfer the basic value of `reference_kind` to the `RefType` enum.
    /// 
    /// # Arguments
    /// The valid arguments are defined in the `reference_kind_literals` module.
    pub fn new(val: u8) -> Option<RefType> {
        use reference_kind_literals::*; 
        match val {
            GET_FIELD => 
                Some(RefType::GetField), 
            GET_STATIC => 
                Some(RefType::GetStatic), 
            PUT_FIELD => 
                Some(RefType::PutField), 
            PUT_STATIC => 
                Some(RefType::PutStatic), 
            INVOKE_STATIC => 
                Some(RefType::InvokeStatic),
            INVOKE_VIRTUAL => 
                Some(RefType::InvokeVirtual), 
            INVOKE_SPECIAL => 
                Some(RefType::InvokeSpecial), 
            NEW_INVOKE_SPECIAL =>
                Some(RefType::NewInvokeSpecial),
            INVOKE_INTERFACE =>
                Some(RefType::InvokeInterface),
            _ => None 
        }
    }
}

impl TryInto<RefType> for u8 {
    type Error = (); 
    fn try_into(self) -> Result<RefType, ()> {
        match RefType::new(self) {
            Some(r) => Ok(r), 
            None => Err(())
        }
    }
}
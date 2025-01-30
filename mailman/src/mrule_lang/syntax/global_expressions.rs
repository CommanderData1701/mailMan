use crate::mrule_lang::syntax::{ServerDO, TriggerExpression, Action};

pub enum GlobalExpressions {
    ServerDefintion(ServerDO),
    MRule(TriggerExpression, Action),
}

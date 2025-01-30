pub enum MailAtrribute {
    Header(String),
    Content(String),
}

pub enum TriggerExpression {
    And(Box<TriggerExpression>, Box<TriggerExpression>),
    Or(Box<TriggerExpression>, Box<TriggerExpression>),
    Not(Box<TriggerExpression>),

    Equals(MailAtrribute, String),
    IsEmpty(MailAtrribute),
    Contains(MailAtrribute, String),
}

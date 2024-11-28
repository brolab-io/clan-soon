use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorMessages {
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Clan is inactive")]
    ClanInactive,
    #[msg("User is not a member of clan or member is inactive ")]
    MemberInactive,
    #[msg("Member is inactive or banned from clan")]
    MemberInactiveOrBanned,
    #[msg("Member is already in a clan")]
    AlreadyMember,
    #[msg("Member is banned from clan")]
    MemberIsBanned,
    #[msg("User is not a member of clan")]
    NotMember,
    #[msg("User is not authorized to perform this action")]
    NotAuthorize,
    #[msg("Leader cannot leave clan")]
    LeaderCannotLeave,
    #[msg("Clan already exists")]
    ClanAlreadyExists,
    #[msg("Name is too long")]
    ClanNameTooLong,
    #[msg("Symbol is too long")]
    ClanSymbolTooLong,
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    #[msg("Invalid master edition")]
    InvalidMasterEdition,
    #[msg("Title is too long")]
    TitleTooLong,
    #[msg("Description is too long")]
    DescriptionTooLong,
    #[msg("Proposal start date must be after end date")]
    ProposalStartAfterEnd,
    #[msg("Proposal start date must be in the future")]
    ProposalStartInPast,
    #[msg("Proposal amount must be greater than 0")]
    ProposalAmountZero,
    #[msg("Proposal already started. Cannot update")]
    ProposalAlreadyStarted,
    #[msg("Proposal not started yet. Cannot vote")]
    ProposalNotStarted,
    #[msg("Proposal already ended. Cannot vote")]
    ProposalAlreadyEnded,
    #[msg("Proposal not passed. Cannot execute")]
    ProposalNotPassed,
    #[msg("Proposal already executed")]
    ProposalAlreadyExecuted,
    #[msg("Not enough power to execute proposal")]
    NotEnoughPower,
    #[msg("Proposal not ended")]
    ProposalNotEnded,
    #[msg("User already claimed")]
    UserClaimed,
}

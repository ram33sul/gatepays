use super::failure_dto::FailureDto;

pub type ResultDto<T> = Result<T, FailureDto>;

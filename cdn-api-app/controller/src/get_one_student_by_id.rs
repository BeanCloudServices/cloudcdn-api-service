use crate::openapi::ToOpenApi;
use chrono::NaiveDate;
use db_postgres::polity_gateway::repository::PolityRepository;
use db_postgres::saint_gateway::repository::SaintRepository;
use db_postgres::student_gateway::repository::StudentRepository;
use domain::usecases::query_one_student_by_id_usecase::{
    QueryOneStudentByIdUsecase, QueryOneStudentByIdUsecaseInteractor,
};
use domain::usecases::query_student_collection_usecase::{
    QueryStudentCollectionUsecaseInputSortField, QueryStudentCollectionUsecaseInteractor,
};
use domain::usecases::student_usecase_shared_models::QueryStudentUsecaseOutput;
use hvcg_academics_openapi_student::models::StudentView;
use uuid::Uuid;

pub(crate) async fn from_uuid(id: Uuid) -> Option<StudentView> {
    // Init dependencies
    // let client = db_postgres::connect().await;
    // let student_repository = StudentRepository { client };

    // let polity_client = db_postgres::connect().await;
    // let polity_repository = PolityRepository {
    //     client: polity_client,
    // };

    // let saint_client = db_postgres::connect().await;
    // let saint_repository = SaintRepository {
    //     client: saint_client,
    // };

    // // Inject dependencies to Interactor and invoke func
    // let query_one_student_usecase_output = QueryOneStudentByIdUsecaseInteractor::new(
    //     student_repository,
    //     polity_repository,
    //     saint_repository,
    // )
    // .execute(id)
    // .await;

    let query_one_student_usecase_output: Option<QueryStudentUsecaseOutput> =
        Option::from(QueryStudentUsecaseOutput {
            id,
            polity_id: None,
            polity_name: Some(String::from("Cần Thơ")),
            polity_location_name: Some(String::from("Tòa Giám Mục Cần Thơ")),
            polity_location_address: Some(String::from("12 Nguyễn Trãi, Ninh Kiều, Cần Thơ")),
            polity_location_email: Some(String::from("binh@sunrise.vn")),
            saint_ids: None,
            christian_name: Option::from(vec![String::from("Phêrô")]),
            title: Some(String::from("PRIEST")),
            first_name: Some("Nguyễn".to_owned()),
            middle_name: Some(String::from("Hữu")),
            last_name: Some(String::from("Chiến")),
            date_of_birth: Some(NaiveDate::from_ymd(1983, 5, 16)),
            place_of_birth: Some(String::from("Trà Vinh")),
            email: Some(String::from("binh@sunrise.vn")),
            phone: Some(String::from("+84 1228019700")),
            undergraduate_school: None,
        });

    query_one_student_usecase_output
        .map(|query_one_student_usecase_output| query_one_student_usecase_output.to_openapi())
}

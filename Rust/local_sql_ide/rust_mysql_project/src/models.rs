use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Empregado {
    pub num_matricula: i32,
    pub nom_empregado: String,
    pub dat_nascimento: NaiveDate,
    pub dsc_endereco: String,
    pub nom_cidade: String,
    pub sig_uf: String,
    pub sex_empregado: String,
    pub val_salario: f64,
    pub num_matricula_supervisor: Option<i32>,
    pub cod_depto: i32,
}

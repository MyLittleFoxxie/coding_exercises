CREATE DATABASE IF NOT EXISTS mydb;
USE mydb;

CREATE TABLE departamento (
    cod_depto INT PRIMARY KEY,
    nom_depto VARCHAR(255),
    num_matricula_gerente INT,
    dat_inicio_gerente DATE
);

CREATE TABLE empregado (
    num_matricula INT PRIMARY KEY,
    nom_empregado VARCHAR(255),
    dat_nascimento DATE,
    dsc_endereco VARCHAR(255),
    nom_cidade VARCHAR(255),
    sig_uf CHAR(2),
    sex_empregado CHAR(1),
    val_salario DOUBLE,
    num_matricula_supervisor INT,
    cod_depto INT
);

ALTER TABLE empregado ADD CONSTRAINT fk_empregado_departamento
FOREIGN KEY (cod_depto) REFERENCES departamento(cod_depto);

ALTER TABLE departamento ADD CONSTRAINT fk_departamento_empregado
FOREIGN KEY (num_matricula_gerente) REFERENCES empregado(num_matricula);

CREATE TABLE local (
    cod_local INT PRIMARY KEY,
    sigla VARCHAR(2),
    nome_cidade VARCHAR(255)
);

CREATE TABLE departamento_local (
    cod_depto INT,
    cod_local INT,
    PRIMARY KEY (cod_depto, cod_local),
    FOREIGN KEY (cod_depto) REFERENCES departamento(cod_depto),
    FOREIGN KEY (cod_local) REFERENCES local(cod_local)
);

CREATE TABLE projeto (
    cod_projeto INT PRIMARY KEY,
    nom_projeto VARCHAR(255),
    nom_local VARCHAR(255),
    cod_depto INT,
    FOREIGN KEY (cod_depto) REFERENCES departamento(cod_depto)
);

CREATE TABLE alocacao (
    num_matricula INT,
    cod_projeto INT,
    num_horas INT,
    PRIMARY KEY (num_matricula, cod_projeto),
    FOREIGN KEY (num_matricula) REFERENCES empregado(num_matricula),
    FOREIGN KEY (cod_projeto) REFERENCES projeto(cod_projeto) 
);

CREATE TABLE dependente (
    num_matricula INT,
    nom_dependente VARCHAR(255),
    sex_dependente VARCHAR(1),
    dsc_parentesco VARCHAR(255),
    PRIMARY KEY (num_matricula, nom_dependente),
    FOREIGN KEY (num_matricula) REFERENCES empregado(num_matricula)
);

-- Inserting into `departamento`
INSERT INTO mydb.departamento (cod_depto, nom_depto, num_matricula_gerente, dat_inicio_gerente)
VALUES (1, 'Desenvolvimento', NULL, '2023-01-01');

-- Inserting into `local`
INSERT INTO mydb.local (cod_local, sigla, nome_cidade)
VALUES (1, 'SP', 'São Paulo');

-- The `empregado` table depends on `departamento`
-- Assuming a gerente exists, if not, you might need to insert without num_matricula_gerente or update it later
INSERT INTO mydb.empregado (num_matricula, nom_empregado, dat_nascimento, dsc_endereco, nom_cidade, sig_uf, sex_empregado, val_salario, num_matricula_supervisor, cod_depto)
VALUES (1, 'João Silva', '1985-02-15', 'Rua Exemplo, 123', 'São Paulo', 'SP', 'M', 5000.00, NULL, 1);

-- Update `departamento` setting `num_matricula_gerente`
UPDATE mydb.departamento SET num_matricula_gerente = 1 WHERE cod_depto = 1;

-- Inserting into `departamento_local`
INSERT INTO mydb.departamento_local (cod_depto, cod_local)
VALUES (1, 1);

-- Inserting into `projeto`
INSERT INTO mydb.projeto (cod_projeto, nom_projeto, nom_local, cod_depto)
VALUES (1, 'Projeto Principal', 'São Paulo', 1);

-- Inserting into `alocacao`
INSERT INTO mydb.alocacao (num_matricula, cod_projeto, num_horas)
VALUES (1, 1, 40);

-- Inserting into `dependente`
INSERT INTO mydb.dependente (num_matricula, nom_dependente, sex_dependente, dsc_parentesco)
VALUES (1, 'Maria Silva', 'F', 'Esposa');

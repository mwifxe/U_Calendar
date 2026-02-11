-- U_Calendar Database Schema
-- PostgreSQL Migration

-- Periodos académicos (I, II, III)
CREATE TABLE IF NOT EXISTS periodos (
                                        id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    numero      VARCHAR(10)  NOT NULL,          -- 'I', 'II', 'III'
    nombre      VARCHAR(100) NOT NULL,          -- 'Período I'
    info        VARCHAR(255) NOT NULL,          -- '7 semanas · 19 ene – 7 mar'
    orden       SMALLINT     NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT now()
    );

-- Semanas dentro de cada período
CREATE TABLE IF NOT EXISTS semanas (
                                       id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    periodo_id      UUID         NOT NULL REFERENCES periodos(id) ON DELETE CASCADE,
    codigo          VARCHAR(20)  NOT NULL UNIQUE,  -- 'p1s1', 'p2s3', etc.
    titulo          VARCHAR(150) NOT NULL,          -- 'Semana 1', 'Semana 7 — PARCIALES'
    fecha           VARCHAR(100) NOT NULL,          -- '19 – 24 ene'
    es_parcial      BOOLEAN      NOT NULL DEFAULT FALSE,
    es_vacaciones   BOOLEAN      NOT NULL DEFAULT FALSE,
    nota_especial   TEXT,
    orden           SMALLINT     NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT now()
    );

-- Materias de parciales (solo para semanas de parciales)
CREATE TABLE IF NOT EXISTS parciales (
                                         id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    semana_id   UUID         NOT NULL REFERENCES semanas(id) ON DELETE CASCADE,
    materia     VARCHAR(150) NOT NULL,
    orden       SMALLINT     NOT NULL DEFAULT 0
    );

-- Actividades evaluadas
CREATE TABLE IF NOT EXISTS actividades (
                                           id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    semana_id   UUID         NOT NULL REFERENCES semanas(id) ON DELETE CASCADE,
    materia     VARCHAR(150) NOT NULL,
    titulo      VARCHAR(255) NOT NULL,
    fecha       VARCHAR(100),
    porcentaje  VARCHAR(20),
    descripcion TEXT,
    clase       VARCHAR(50)  NOT NULL DEFAULT '',  -- CSS class: 'matematica', 'filosofia', etc.
    orden       SMALLINT     NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT now()
    );

-- Notas personales por actividad
CREATE TABLE IF NOT EXISTS notas (
                                     id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    actividad_id  UUID NOT NULL REFERENCES actividades(id) ON DELETE CASCADE,
    contenido     TEXT NOT NULL DEFAULT '',
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(actividad_id)
    );

-- Índices para queries frecuentes
CREATE INDEX IF NOT EXISTS idx_semanas_periodo     ON semanas(periodo_id, orden);
CREATE INDEX IF NOT EXISTS idx_actividades_semana  ON actividades(semana_id, orden);
CREATE INDEX IF NOT EXISTS idx_notas_actividad     ON notas(actividad_id);
CREATE INDEX IF NOT EXISTS idx_parciales_semana    ON parciales(semana_id, orden);
-- U_Calendar Seed Data
-- Matches the original hardcoded calendar from index.html

-- ============================================================
-- PERÍODO I
-- ============================================================
INSERT INTO periodos (id, numero, nombre, info, orden) VALUES
    ('a0000000-0000-0000-0000-000000000001', 'I', 'Período I', '7 semanas · 19 ene – 7 mar', 1);

INSERT INTO semanas (id, periodo_id, codigo, titulo, fecha, es_parcial, es_vacaciones, nota_especial, orden) VALUES
                                                                                                                 ('b0000000-0000-0000-0000-000000000001', 'a0000000-0000-0000-0000-000000000001', 'p1s1', 'Semana 1', '19 – 24 ene', FALSE, FALSE, NULL, 1),
                                                                                                                 ('b0000000-0000-0000-0000-000000000002', 'a0000000-0000-0000-0000-000000000001', 'p1s2', 'Semana 2', '26 – 31 ene', FALSE, FALSE, NULL, 2),
                                                                                                                 ('b0000000-0000-0000-0000-000000000003', 'a0000000-0000-0000-0000-000000000001', 'p1s3', 'Semana 3', '2 – 7 feb', FALSE, FALSE, NULL, 3),
                                                                                                                 ('b0000000-0000-0000-0000-000000000004', 'a0000000-0000-0000-0000-000000000001', 'p1s4', 'Semana 4', '9 – 14 feb', FALSE, FALSE, NULL, 4),
                                                                                                                 ('b0000000-0000-0000-0000-000000000005', 'a0000000-0000-0000-0000-000000000001', 'p1s5', 'Semana 5', '16 – 21 feb', FALSE, FALSE, NULL, 5),
                                                                                                                 ('b0000000-0000-0000-0000-000000000006', 'a0000000-0000-0000-0000-000000000001', 'p1s6', 'Semana 6', '23 – 28 feb', FALSE, FALSE, 'Foro Dignidad Humana sigue habilitado hasta el 24/02', 6),
                                                                                                                 ('b0000000-0000-0000-0000-000000000007', 'a0000000-0000-0000-0000-000000000001', 'p1s7', 'Semana 7 — PARCIALES', '2 – 7 mar', TRUE, FALSE, NULL, 7);

-- Actividades Período I
INSERT INTO actividades (semana_id, materia, titulo, fecha, porcentaje, clase, orden) VALUES
                                                                                          -- Semana 3
                                                                                          ('b0000000-0000-0000-0000-000000000003', 'Desarrollo Personal', 'Actividad 1 – El Bien', '03/02/26', '20%', '', 1),
                                                                                          ('b0000000-0000-0000-0000-000000000003', 'Matemática I', 'Laboratorio Corto 1', '05/02/26', '20%', 'matematica', 2),
                                                                                          -- Semana 4
                                                                                          ('b0000000-0000-0000-0000-000000000004', 'Desarrollo Personal', 'Actividad 2 – Sentido de la vida', '10/02/26', '20%', '', 1),
                                                                                          ('b0000000-0000-0000-0000-000000000004', 'Desarrollo Personal', 'Actividad 3 – Foro Dignidad Humana', '10 al 24/02', '10%', '', 2),
                                                                                          ('b0000000-0000-0000-0000-000000000004', 'Filosofía', 'Examen – Introducción a la Filosofía', 'Jueves 12/02/26', '', 'filosofia', 3),
                                                                                          -- Semana 5
                                                                                          ('b0000000-0000-0000-0000-000000000005', 'Matemática I', 'Laboratorio Corto 2', '20/02/26', '20%', 'matematica', 1),
                                                                                          -- Semana 6
                                                                                          ('b0000000-0000-0000-0000-000000000006', 'Matemática I', 'Guía de ejercicios', '27/02/26', '10%', 'matematica', 1);

-- Parciales Período I
INSERT INTO parciales (semana_id, materia, orden) VALUES
                                                      ('b0000000-0000-0000-0000-000000000007', 'Desarrollo Personal', 1),
                                                      ('b0000000-0000-0000-0000-000000000007', 'Matemática I', 2),
                                                      ('b0000000-0000-0000-0000-000000000007', 'Teología', 3),
                                                      ('b0000000-0000-0000-0000-000000000007', 'Filosofía', 4),
                                                      ('b0000000-0000-0000-0000-000000000007', 'Técnicas de Redacción', 5);

-- ============================================================
-- PERÍODO II
-- ============================================================
INSERT INTO periodos (id, numero, nombre, info, orden) VALUES
    ('a0000000-0000-0000-0000-000000000002', 'II', 'Período II', '6 semanas · 9 mar – 25 abr', 2);

INSERT INTO semanas (id, periodo_id, codigo, titulo, fecha, es_parcial, es_vacaciones, nota_especial, orden) VALUES
                                                                                                                 ('b0000000-0000-0000-0000-000000000008', 'a0000000-0000-0000-0000-000000000002', 'p2s1', 'Semana 1', '9 – 14 mar', FALSE, FALSE, NULL, 1),
                                                                                                                 ('b0000000-0000-0000-0000-000000000009', 'a0000000-0000-0000-0000-000000000002', 'p2s2', 'Semana 2', '16 – 21 mar', FALSE, FALSE, NULL, 2),
                                                                                                                 ('b0000000-0000-0000-0000-000000000010', 'a0000000-0000-0000-0000-000000000002', 'p2s3', 'Semana 3', '23 – 28 mar', FALSE, FALSE, NULL, 3),
                                                                                                                 ('b0000000-0000-0000-0000-000000000011', 'a0000000-0000-0000-0000-000000000002', 'p2sv', 'Vacaciones de Semana Santa', '30 mar – 5 abr', FALSE, TRUE, NULL, 4),
                                                                                                                 ('b0000000-0000-0000-0000-000000000012', 'a0000000-0000-0000-0000-000000000002', 'p2s4', 'Semana 4', '6 – 11 abr', FALSE, FALSE, NULL, 5),
                                                                                                                 ('b0000000-0000-0000-0000-000000000013', 'a0000000-0000-0000-0000-000000000002', 'p2s5', 'Semana 5', '13 – 18 abr', FALSE, FALSE, NULL, 6),
                                                                                                                 ('b0000000-0000-0000-0000-000000000014', 'a0000000-0000-0000-0000-000000000002', 'p2s6', 'Semana 6 — PARCIALES', '20 – 25 abr', TRUE, FALSE, NULL, 7);

-- Actividades Período II
INSERT INTO actividades (semana_id, materia, titulo, fecha, porcentaje, clase, orden) VALUES
                                                                                          -- Semana 2
                                                                                          ('b0000000-0000-0000-0000-000000000009', 'Desarrollo Personal', 'Actividad 1 – Ética y Moral', '17/03/26', '20%', '', 1),
                                                                                          ('b0000000-0000-0000-0000-000000000009', 'Matemática I', 'Laboratorio Corto 3', '20/03/26', '20%', 'matematica', 2),
                                                                                          -- Semana 3
                                                                                          ('b0000000-0000-0000-0000-000000000010', 'Desarrollo Personal', 'Actividad 2 – Foro El Efecto Padre', '24/03/26', '10%', '', 1),
                                                                                          -- Semana 4
                                                                                          ('b0000000-0000-0000-0000-000000000012', 'Desarrollo Personal', 'Actividad 3 – Educar los sentimientos', '07/04/26', '20%', '', 1),
                                                                                          -- Semana 5
                                                                                          ('b0000000-0000-0000-0000-000000000013', 'Matemática I', 'Laboratorio Corto 4', '16/04/26', '20%', 'matematica', 1),
                                                                                          ('b0000000-0000-0000-0000-000000000013', 'Matemática I', 'Guía de ejercicios', '17/04/26', '10%', 'matematica', 2);

-- Parciales Período II
INSERT INTO parciales (semana_id, materia, orden) VALUES
                                                      ('b0000000-0000-0000-0000-000000000014', 'Desarrollo Personal', 1),
                                                      ('b0000000-0000-0000-0000-000000000014', 'Matemática I', 2),
                                                      ('b0000000-0000-0000-0000-000000000014', 'Teología', 3),
                                                      ('b0000000-0000-0000-0000-000000000014', 'Filosofía', 4),
                                                      ('b0000000-0000-0000-0000-000000000014', 'Técnicas de Redacción', 5);

-- ============================================================
-- PERÍODO III
-- ============================================================
INSERT INTO periodos (id, numero, nombre, info, orden) VALUES
    ('a0000000-0000-0000-0000-000000000003', 'III', 'Período III', '6 semanas · 27 abr – 6 jun', 3);

INSERT INTO semanas (id, periodo_id, codigo, titulo, fecha, es_parcial, es_vacaciones, nota_especial, orden) VALUES
                                                                                                                 ('b0000000-0000-0000-0000-000000000015', 'a0000000-0000-0000-0000-000000000003', 'p3s1', 'Semana 1', '27 abr – 2 may', FALSE, FALSE, NULL, 1),
                                                                                                                 ('b0000000-0000-0000-0000-000000000016', 'a0000000-0000-0000-0000-000000000003', 'p3s2', 'Semana 2', '4 – 9 may', FALSE, FALSE, NULL, 2),
                                                                                                                 ('b0000000-0000-0000-0000-000000000017', 'a0000000-0000-0000-0000-000000000003', 'p3s3', 'Semana 3', '11 – 16 may', FALSE, FALSE, NULL, 3),
                                                                                                                 ('b0000000-0000-0000-0000-000000000018', 'a0000000-0000-0000-0000-000000000003', 'p3s4', 'Semana 4', '18 – 23 may', FALSE, FALSE, NULL, 4),
                                                                                                                 ('b0000000-0000-0000-0000-000000000019', 'a0000000-0000-0000-0000-000000000003', 'p3s5', 'Semana 5', '25 – 30 may', FALSE, FALSE, NULL, 5),
                                                                                                                 ('b0000000-0000-0000-0000-000000000020', 'a0000000-0000-0000-0000-000000000003', 'p3s6', 'Semana 6 — PARCIALES FINALES', '1 – 6 jun', TRUE, FALSE, NULL, 6);

-- Actividades Período III
INSERT INTO actividades (semana_id, materia, titulo, fecha, porcentaje, clase, orden) VALUES
                                                                                          -- Semana 2
                                                                                          ('b0000000-0000-0000-0000-000000000016', 'Matemática I', 'Laboratorio Corto 5', '08/05/26', '20%', 'matematica', 1),
                                                                                          -- Semana 3
                                                                                          ('b0000000-0000-0000-0000-000000000017', 'Desarrollo Personal', 'Actividad 1 – Retos de la familia actual', '12/05/26', '20%', '', 1),
                                                                                          -- Semana 4
                                                                                          ('b0000000-0000-0000-0000-000000000018', 'Desarrollo Personal', 'Actividad 2 – La Sociedad', '19/05/26', '15%', '', 1),
                                                                                          -- Semana 5
                                                                                          ('b0000000-0000-0000-0000-000000000019', 'Desarrollo Personal', 'Actividad 3 – Entrenamiento ético', '26/05/26', '15%', '', 1),
                                                                                          ('b0000000-0000-0000-0000-000000000019', 'Matemática I', 'Laboratorio Corto 6', '28/05/26', '20%', 'matematica', 2),
                                                                                          ('b0000000-0000-0000-0000-000000000019', 'Matemática I', 'Guía de ejercicios', '29/05/26', '10%', 'matematica', 3);

-- Parciales Período III
INSERT INTO parciales (semana_id, materia, orden) VALUES
                                                      ('b0000000-0000-0000-0000-000000000020', 'Desarrollo Personal', 1),
                                                      ('b0000000-0000-0000-0000-000000000020', 'Matemática I', 2),
                                                      ('b0000000-0000-0000-0000-000000000020', 'Teología', 3),
                                                      ('b0000000-0000-0000-0000-000000000020', 'Filosofía', 4),
                                                      ('b0000000-0000-0000-0000-000000000020', 'Técnicas de Redacción', 5);
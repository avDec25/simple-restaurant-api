CREATE DATABASE IF NOT EXISTS  restaurant;

USE restaurant;

CREATE TABLE IF NOT EXISTS table_items (
  item_id INT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
  table_number INT UNSIGNED NOT NULL,
  item_name VARCHAR(127) NOT NULL,
  prepare_minutes INT UNSIGNED NOT NULL,
  ordered_on DATETIME NOT NULL
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;

CREATE INDEX index_on_table_number ON table_items (table_number);
CREATE DATABASE restaurant;

USE restaurant;

CREATE TABLE table_items (
  item_id INT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
  table_number INT UNSIGNED NOT NULL,
  item_name VARCHAR(25) NOT NULL,
  prepare_minutes INT UNSIGNED NOT NULL,
  ordered_on DATETIME NOT NULL
);
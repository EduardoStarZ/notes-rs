-- This file should undo anything in `up.sql`
CREATE TABLE `notes`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL,
	`content` TEXT NOT NULL
);

CREATE TABLE `users`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL
);

DROP TABLE IF EXISTS `note`;
DROP TABLE IF EXISTS `user`;

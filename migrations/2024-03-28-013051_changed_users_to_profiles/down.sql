-- This file should undo anything in `up.sql`
ALTER TABLE `note` DROP COLUMN `profile_id`;
ALTER TABLE `note` ADD COLUMN `user_id` INTEGER NOT NULL;

CREATE TABLE `user`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL,
	`active` BOOL NOT NULL
);

DROP TABLE IF EXISTS `profile`;

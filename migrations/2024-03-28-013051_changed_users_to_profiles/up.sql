-- Your SQL goes here
ALTER TABLE `note` DROP COLUMN `user_id`;
ALTER TABLE `note` ADD COLUMN `profile_id` INTEGER NOT NULL;

DROP TABLE IF EXISTS `user`;
CREATE TABLE `profile`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL,
	`active` BOOL NOT NULL
);


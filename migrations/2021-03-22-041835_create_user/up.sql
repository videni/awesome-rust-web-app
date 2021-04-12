CREATE TABLE user (
    `user_id`  UUID PRIMARY KEY DEFAULT uuid_generate_v4() COMMENT 'User ID',
    `username` varchar(40) DEFAULT NULL COMMENT 'User Login',
    `password` varchar(255) NOT NULL COMMENT 'User Password',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'User Created Time',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'User Updated Time',
    `email` varchar(128) DEFAULT NULL COMMENT 'User Email',
    `failures_num` smallint DEFAULT '0' COMMENT 'Failure Number',
    `first_failed_at` timestamp NULL DEFAULT NULL COMMENT 'First Failure',
    `lock_expires_at` timestamp NULL DEFAULT NULL COMMENT 'Expiration Lock Dates',
    `enabled` tinyint NOT NULL DEFAULT '1' COMMENT 'User Is enabled',
    `salt` varchar(255) DEFAULT NULL COMMENT 'Salt to encode password',
    PRIMARY KEY (`user_id`),
    UNIQUE KEY `USER_USERNAME` (`username`)
)
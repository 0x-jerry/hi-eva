CREATE TABLE IF NOT EXISTS
  `prompt_config` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `createdDate` DATETIME NOT NULL,
    `updatedDate` DATETIME NOT NULL,
    -- 
    `name` VARCHAR(64) NOT NULL,
    `icon` VARCHAR(255) NOT NULL,
    `prompt` text NOT NULL,
    `isBuiltin` INTEGER DEFAULT 0
  );

CREATE TABLE IF NOT EXISTS
  `chat_history` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `createdDate` DATETIME NOT NULL,
    `updatedDate` DATETIME NOT NULL,
    -- 
    `title` VARCHAR(255) NOT NULL,
    `model` VARCHAR(64) NOT NULL, -- model name
    `promptConfigId` VARCHAR(64) NOT NULL
  );

CREATE TABLE IF NOT EXISTS
  `chat_history_msg` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `createdDate` DATETIME NOT NULL,
    `updatedDate` DATETIME NOT NULL,
    -- 
    `chatHistoryId` INTEGER NOT NULL,
    `content` text NOT NULL,
    `role` VARCHAR(64) NOT NULL
  );
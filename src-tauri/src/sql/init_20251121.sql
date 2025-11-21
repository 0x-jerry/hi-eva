CREATE TABLE IF NOT EXISTS
  `endpoint_config` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `createdDate` DATETIME NOT NULL,
    `updatedDate` DATETIME NOT NULL,
    -- 
    `isBuiltin` INTEGER DEFAULT 0,
    `name` VARCHAR(64),
    `baseUrl` VARCHAR(255),
    `apiKey` VARCHAR(255),
    `model` VARCHAR(64)
  );

CREATE UNIQUE INDEX `endpoint_config_index_id` ON `endpoint_config` (`id`);
CREATE UNIQUE INDEX `endpoint_config_index_updatedDate` ON `endpoint_config` (`updatedDate`);

CREATE TABLE IF NOT EXISTS
  `prompt_config` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `createdDate` DATETIME NOT NULL,
    `updatedDate` DATETIME NOT NULL,
    -- 
    `name` VARCHAR(64),
    `icon` VARCHAR(255),
    `prompt` text,
    `endpointId` INTEGER,
    `isBuiltin` INTEGER DEFAULT 0
  );

CREATE UNIQUE INDEX `prompt_config_index_id` ON `prompt_config` (`id`);
CREATE UNIQUE INDEX `prompt_config_index_updatedDate` ON `prompt_config` (`updatedDate`);

CREATE TABLE IF NOT EXISTS
  `chat_history` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `createdDate` DATETIME NOT NULL,
    `updatedDate` DATETIME NOT NULL,
    -- 
    `name` VARCHAR(255) NOT NULL
  );

CREATE UNIQUE INDEX `chat_history_index_id` ON `chat_history` (`id`);
CREATE UNIQUE INDEX `chat_history_index_updatedDate` ON `chat_history` (`updatedDate`);

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

CREATE UNIQUE INDEX `chat_history_msg_index_id` ON `chat_history_msg` (`id`);
CREATE UNIQUE INDEX `chat_history_msg_index_updatedDate` ON `chat_history_msg` (`updatedDate`);

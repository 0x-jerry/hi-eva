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

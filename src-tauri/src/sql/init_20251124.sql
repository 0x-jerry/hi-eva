CREATE TABLE IF NOT EXISTS
  `cache` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `createdDate` DATETIME NOT NULL,
    `updatedDate` DATETIME NOT NULL,
    -- 
    `expiredIn` INTEGER, -- seconds
    `key` VARCHAR(255) UNIQUE NOT NULL,
    `value` text
  );

CREATE UNIQUE INDEX `cache_index_id` ON `cache` (`id`);
CREATE UNIQUE INDEX `cache_index_key` ON `cache` (`key`);

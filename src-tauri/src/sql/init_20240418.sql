CREATE TABLE IF NOT EXISTS
  `selection` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `createdDate` DATETIME NOT NULL,
    `updatedDate` DATETIME NOT NULL,
    -- 
    `selected` text,
    `promptName` VARCHAR(255)
  );
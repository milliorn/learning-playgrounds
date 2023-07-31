CREATE TABLE person
(
PersonID INT,
PersonName varchar(100),
City VARCHAR(50),
Country VARCHAR(50)
)

ALTER TABLE person
ADD COLUMN yearofbirth DATE

ALTER TABLE person
DROP COLUMN Country
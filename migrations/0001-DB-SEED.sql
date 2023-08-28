INSERT INTO products (id, product_name, product_description, product_color) VALUES
('c607a4f8-235b-4625-9b1f-7a30bb3e6e45', 'Men''s Casual Shirt', 'Comfortable and stylish shirt', 'Blue'),
('4317af35-8e42-4364-be35-5e085d11c59b', 'Women''s Floral Dress', 'Elegant dress with a floral pattern', 'Pink'),
('27f937bc-04ff-435b-b966-1b2ab935d46b', 'Men''s Leather Jacket', 'Classic jacket made from genuine leather', 'Black'),
('950002c4-33a0-41f2-bd05-3762dd831f01', 'Women''s Skinny Jeans', 'Slim-fit jeans for a fashionable look', 'Denim'),
('c6219abb-6c93-40e4-bb7e-60b16f6e2ae4', 'Men''s Sports Shoes', 'Lightweight and flexible athletic shoes', 'Red'),
('701c0506-6478-4a51-a5ec-e8fe30fc8b74', 'Women''s Winter Coat', 'Warm and cozy coat for cold weather', 'Beige'),
('1e434335-6cf1-4040-b6f5-57a8dce7b7f5', 'Men''s Polo T-Shirt', 'Classic polo t-shirt for a casual look', 'White'),
('5efb0af6-830c-4b5c-baff-05bef02cb874', 'Women''s Knit Sweater', 'Soft and warm sweater for chilly days', 'Gray'),
('cb15e499-f269-42c8-9dd5-6d1bf618ae09', 'Men''s Formal Suit', 'Elegant suit for formal occasions', 'Navy'),
('80107fa0-7ea8-41a7-b2d5-d990bff639e0', 'Women''s Ankle Boots', 'Stylish boots for a trendy look', 'Brown'),
('3537a406-1dc5-43e8-9c79-e351d9cc1231', 'Men''s Hooded Sweatshirt', 'Comfortable sweatshirt with a hood', 'Black'),
('0e66d39c-dbc2-47ae-b722-80b136754715', 'Women''s Summer Top', 'Lightweight top for hot summer days', 'Yellow'),
('2e3a44ab-65cd-4f52-8756-8a3b29c5c527', 'Men''s Cargo Shorts', 'Durable shorts with multiple pockets', 'Khaki'),
('8b6eee28-9978-4376-92d1-2e6af20f1bf8', 'Women''s High Heels', 'Classy high-heeled shoes for special events', 'Black'),
('bfec1dd0-db80-4ee7-b9b2-d2e1deab00a3', 'Men''s Plaid Shirt', 'Fashionable shirt with a plaid pattern', 'Green');

INSERT INTO categories (id, category_reference, category_name, category_description) VALUES
('7413c90f-a5e6-41e8-a952-f7f2cb6f4a93', 'woman', 'Woman''s category', 'Category that holds every single female product'),
('f7651a25-66b0-4083-a0ff-ca93a7ad2217', 'man', 'Man''s category', 'Category that holds every single male product');

INSERT INTO categories_products (id, category_id, product_id) VALUES
('c4257eac-f08b-466a-b13b-cfd893c5a1b9', '7413c90f-a5e6-41e8-a952-f7f2cb6f4a93', '4317af35-8e42-4364-be35-5e085d11c59b'),
('a2eb7cd6-6bfc-46f7-97c8-355a9ff271d2', '7413c90f-a5e6-41e8-a952-f7f2cb6f4a93', '950002c4-33a0-41f2-bd05-3762dd831f01'),
('8e60196a-98cb-48bc-a12a-9388033ce444', '7413c90f-a5e6-41e8-a952-f7f2cb6f4a93', '701c0506-6478-4a51-a5ec-e8fe30fc8b74'),
('edfb9495-f3c0-4232-b44d-24c31d1ea04e', '7413c90f-a5e6-41e8-a952-f7f2cb6f4a93', '701c0506-6478-4a51-a5ec-e8fe30fc8b74'),
('cb91f59d-785b-4499-8dbd-1308626acfef', '7413c90f-a5e6-41e8-a952-f7f2cb6f4a93', '5efb0af6-830c-4b5c-baff-05bef02cb874'),
('2cb36cd6-3af9-4e1b-893c-eb6b5e31c9bb', '7413c90f-a5e6-41e8-a952-f7f2cb6f4a93', '80107fa0-7ea8-41a7-b2d5-d990bff639e0'),
('8e8fd63e-0a2d-4c11-b5b6-e4f4d1fe5f1b', '7413c90f-a5e6-41e8-a952-f7f2cb6f4a93', '0e66d39c-dbc2-47ae-b722-80b136754715'),
('c91a3378-6441-4d66-90d0-5178923a93b0', '7413c90f-a5e6-41e8-a952-f7f2cb6f4a93', '8b6eee28-9978-4376-92d1-2e6af20f1bf8'),
('3e70f2c2-5bfe-4bdb-8c80-679937ec60d6', 'f7651a25-66b0-4083-a0ff-ca93a7ad2217', 'c607a4f8-235b-4625-9b1f-7a30bb3e6e45'),
('3f466e2e-c369-4450-9a4c-303b7e11cfdf', 'f7651a25-66b0-4083-a0ff-ca93a7ad2217', '27f937bc-04ff-435b-b966-1b2ab935d46b'),
('02712c06-ccfb-4415-bb75-365f0bbcf966', 'f7651a25-66b0-4083-a0ff-ca93a7ad2217', 'c6219abb-6c93-40e4-bb7e-60b16f6e2ae4'),
('b9b9c0b4-2c23-4500-b68c-0023f9e29e25', 'f7651a25-66b0-4083-a0ff-ca93a7ad2217', '1e434335-6cf1-4040-b6f5-57a8dce7b7f5'),
('d6d65c04-9653-4a27-80fc-09f3feb054b3', 'f7651a25-66b0-4083-a0ff-ca93a7ad2217', 'cb15e499-f269-42c8-9dd5-6d1bf618ae09'),
('a9710f0e-2644-4054-a8da-c129a7bd39cc', 'f7651a25-66b0-4083-a0ff-ca93a7ad2217', '3537a406-1dc5-43e8-9c79-e351d9cc1231'),
('2b917d1b-c8e9-4860-97c8-41476c2ee31a', 'f7651a25-66b0-4083-a0ff-ca93a7ad2217', '2e3a44ab-65cd-4f52-8756-8a3b29c5c527'),
('ca52ec37-f600-4fcb-af1e-87a7176ae57e', 'f7651a25-66b0-4083-a0ff-ca93a7ad2217', 'bfec1dd0-db80-4ee7-b9b2-d2e1deab00a3');

INSERT INTO inventories (id, inventory_name, inventory_reference) VALUES 
('26c943fa-c247-405d-8bbb-b649efbdd487', 'Global inventory', 'global');

INSERT INTO inventories_products (inventory_id, product_id, allocation, id) VALUES
('26c943fa-c247-405d-8bbb-b649efbdd487', 'c607a4f8-235b-4625-9b1f-7a30bb3e6e45', 100, '7ad52583-3c2f-4d8a-a6b4-abc386ad86e6'),
('26c943fa-c247-405d-8bbb-b649efbdd487', '4317af35-8e42-4364-be35-5e085d11c59b', 100, 'bac84ea4-2997-48c4-8569-5b233530cd16'),
('26c943fa-c247-405d-8bbb-b649efbdd487', '27f937bc-04ff-435b-b966-1b2ab935d46b', 100, '6e917f18-6472-44d7-bfc2-4d60b5190a5a'),
('26c943fa-c247-405d-8bbb-b649efbdd487', '950002c4-33a0-41f2-bd05-3762dd831f01', 100, '182f4cdf-8840-4fe4-ba22-272fb52f0605'),
('26c943fa-c247-405d-8bbb-b649efbdd487', 'c6219abb-6c93-40e4-bb7e-60b16f6e2ae4', 100, '3ee6db69-8ed3-4c6b-ac9b-c50d4b37532c'),
('26c943fa-c247-405d-8bbb-b649efbdd487', '701c0506-6478-4a51-a5ec-e8fe30fc8b74', 100, 'b07edf27-b0b0-42ca-b506-a8b29e111047'),
('26c943fa-c247-405d-8bbb-b649efbdd487', '1e434335-6cf1-4040-b6f5-57a8dce7b7f5', 100, '8673165f-49fd-4f71-a779-dd7cf4536d80'),
('26c943fa-c247-405d-8bbb-b649efbdd487', '5efb0af6-830c-4b5c-baff-05bef02cb874', 100, '94fb7559-0601-4bf5-a698-c18db64f418f'),
('26c943fa-c247-405d-8bbb-b649efbdd487', 'cb15e499-f269-42c8-9dd5-6d1bf618ae09', 100, '5c8fc08d-4569-4b8b-b003-15665661265e'),
('26c943fa-c247-405d-8bbb-b649efbdd487', '80107fa0-7ea8-41a7-b2d5-d990bff639e0', 100, 'f2f5bda4-ad6c-4d36-88e7-a7b9ab6d7159'),
('26c943fa-c247-405d-8bbb-b649efbdd487', '3537a406-1dc5-43e8-9c79-e351d9cc1231', 100, 'd4e73d39-3b51-432f-8aad-889747032679'),
('26c943fa-c247-405d-8bbb-b649efbdd487', '0e66d39c-dbc2-47ae-b722-80b136754715', 100, '39a23d4d-5e74-4117-93ab-d89a3eb9d8c4'),
('26c943fa-c247-405d-8bbb-b649efbdd487', '2e3a44ab-65cd-4f52-8756-8a3b29c5c527', 100, '3c76f0d2-511e-4c0a-ac1b-74a61ac6b5d1'),
('26c943fa-c247-405d-8bbb-b649efbdd487', '8b6eee28-9978-4376-92d1-2e6af20f1bf8', 100, 'a634b69c-ba2c-4079-894a-707ecfd693f2'),
('26c943fa-c247-405d-8bbb-b649efbdd487', 'bfec1dd0-db80-4ee7-b9b2-d2e1deab00a3', 100, 'a38492a4-1a16-40d2-b4a7-1fd61a2a4321');

INSERT INTO portal_users (id, email, first_name, last_name, password, roles) VALUES
('60ba424e-caa9-40c8-80bd-6172a3bd49fe', 'root@root.com', 'root', 'root', '$argon2id$v=19$m=19456,t=2,p=1$Z8HzSqALOSpvxlP9igLchQ$Gf30BEPioLBRb9e6/7SlK/8JP3dvngOu3bFN4MFIoMc', '{ADMIN, READER, EDITOR}');

INSERT INTO pricebooks (id, pricebook_name, pricebook_reference, pricebook_currencty_code) VALUES 
('57c19938-4253-4d25-a98f-5d8a75d78051', 'Global pricebook', 'global-pricebook', 'USD');

INSERT INTO pricebooks_products (id, pricebook_id, product_id, price) VALUES 
('44d592bc-fd99-4d57-a44d-a6284fb63da0', '57c19938-4253-4d25-a98f-5d8a75d78051', 'c607a4f8-235b-4625-9b1f-7a30bb3e6e45', 100.0);

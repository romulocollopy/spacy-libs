BEGIN;

INSERT INTO public.sku(
	id, quantity, name, data)
	VALUES ('62116e1e-538f-4401-81d1-7c2f5694d596', 3, 'ball', '{"foo": "bar"}');

COMMIT;

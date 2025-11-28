test(async () => {
	const form = queryFormById(rowform)
	assertNotUndefined(form)


	const [_table, originalRows] = queryTableByTbodyId(rowtablebody, true);


	setInputValueByid(form, 'masodik', 'Test item')
	setInputValueByid(form, 'elso', 'Test item1')


	await triggerSubmit(form)

	const [_, newRows] = queryTableByTbodyId(rowtablebody, true);
	const errorMessageThird = getErrorFieldContentByInputId(form, 'harmadik');
	assertNotEmptyString(errorMessageThird, `3. beviteli mező erorüzenete nem jelenik meg`);
	return assertEquals(originalRows, newRows, "Sort fűzött hozzá a táblázathoz a validáció ellenére");
})
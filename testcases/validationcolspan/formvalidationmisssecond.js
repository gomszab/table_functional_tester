test(async () => {
	const form = queryFormById(colform)
	assertNotUndefined(form)


	const [_table, originalRows] = queryTableByTbodyId(coltablebody, true);


	setInputValueByid(form, 'elso', 'Test item')
	setInputValueByid(form, 'harmadik', 'Test item1')


	await triggerSubmit(form)

	const [_, newRows] = queryTableByTbodyId(coltablebody, true);
	const errorMessageMasodik = getErrorFieldContentByInputId(form, 'masodik');
	assertNotEmptyString(errorMessageMasodik, `2. beviteli mező erorüzenete nem jelenik meg`);
	return assertEquals(originalRows, newRows, "Sort fűzött hozzá a táblázathoz a validáció ellenére");
})
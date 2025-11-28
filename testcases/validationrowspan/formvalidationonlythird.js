test(async () => {
	const form = queryFormById(rowform)
	assertNotUndefined(form)


	const [_table, originalRows] = queryTableByTbodyId(rowtablebody, true);


	setInputValueByid(form, 'harmadik', 'Test item1')


	await triggerSubmit(form)

	const [_, newRows] = queryTableByTbodyId(rowtablebody, true);
	const errorMessageFirst = getErrorFieldContentByInputId(form, 'elso');
	assertNotEmptyString(errorMessageFirst);
	const errorMessageSecond = getErrorFieldContentByInputId(form, 'masodik');
	assertNotEmptyString(errorMessageSecond);
	return assertEquals(originalRows, newRows, "Sort fűzött hozzá a táblázathoz a validáció ellenére");
})
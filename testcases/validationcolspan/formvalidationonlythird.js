test(async () => {
	const form = queryFormById(colform)
	assertNotUndefined(form)


	const [_table, originalRows ]= queryTableByTbodyId(coltablebody, true);


	setInputValueByid(form, 'harmadik','Test item1')


	await triggerSubmit(form)
	
	const [_, newRows ]= queryTableByTbodyId(coltablebody, true);
    const errorMessageFirst = getErrorFieldContentByInputId(form, 'elso');
    assertNotEmptyString(errorMessageFirst);
	const errorMessageSecond = getErrorFieldContentByInputId(form, 'masodik');
    assertNotEmptyString(errorMessageSecond);
	return assertEquals(originalRows, newRows, "Sort fűzött hozzá a táblázathoz a validáció ellenére");
})
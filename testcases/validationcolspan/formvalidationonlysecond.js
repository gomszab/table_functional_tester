test(async () => {
	const form = queryFormById(colform)
	assertNotUndefined(form)


	const [_table, originalRows ]= queryTableByTbodyId(coltablebody, true);


	setInputValueByid(form, 'masodik','Test item1')


	await triggerSubmit(form)
	
	const [_, newRows ]= queryTableByTbodyId(coltablebody, true);
    const errorMessageFirst = getErrorFieldContentByInputId(form, 'elso');
    assertNotEmptyString(errorMessageFirst, `1. beviteli mező erorüzenete nem jelenik meg`);
	const errorMessageThird = getErrorFieldContentByInputId(form, 'harmadik');
    assertNotEmptyString(errorMessageThird, `3. beviteli mező erorüzenete nem jelenik meg`);
	return assertEquals(originalRows, newRows, "Sort fűzött hozzá a táblázathoz a validáció ellenére");
})
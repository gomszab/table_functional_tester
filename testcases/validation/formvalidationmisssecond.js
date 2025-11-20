test(async () => {
	const form = queryFormById('htmlform')
	assertNotUndefined(form)


	const [_table, originalRows ]= queryTableByTbodyId('htmltbody', true);


	setInputValueByid(form, 'elso','Test item')
	setInputValueByid(form, 'harmadik','Test item1')


	await triggerSubmit(form)
	
	const [_, newRows ]= queryTableByTbodyId('htmltbody', true);
    const errorMessageMasodik = getErrorFieldContentByInputId(form, 'masodik');
    assertNotEmptyString(errorMessageMasodik);
	return assertEquals(originalRows, newRows, "Nem fuzott hozza sort");
})
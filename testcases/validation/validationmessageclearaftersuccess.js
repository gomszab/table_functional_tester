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
    assertEquals(originalRows, newRows, "Nem fuzott hozza sort");
    setInputValueByid(form, 'masodik', 'test item 2');
    await triggerSubmit(form)
    const elemList = ['elso', 'masodik', 'harmadik'];
    elemList.map(elem => 
        getErrorFieldContentByInputId(form, elem) 
    ).forEach(element => {
        assertEmptyString(element);
    });
    const [__, newRowsAgain ]= queryTableByTbodyId('htmltbody', true);
	return assertEquals(originalRows+1, newRowsAgain, "Nem fuzott hozza sort");;
})
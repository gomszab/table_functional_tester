test(async () => {
	const form = queryFormById(rowform)
	assertNotUndefined(form)


	const [_table, originalRows ]= queryTableByTbodyId(rowtablebody, true);
    

	setInputValueByid(form, 'elso','Test item 1')
	setInputValueByid(form, 'masodik','Test item 2')
	setInputValueByid(form, 'harmadik','Test item 3')



	await triggerSubmit(form)
	
	const [_, newRows ]= queryTableByTbodyId(rowtablebody, true);
	assertEquals(originalRows+1, newRows, "Nem fuzott hozza sort");
    const lastRow = getLastRowFromTableByTbodyId(rowtablebody);
    assertEquals("Test item 1", lastRow[0].content, "A hozzáadott táblázathoz hozzáadott elem 1. sor 1. oszlopának tartalma nem jó");
    assertEquals("Test item 2", lastRow[1].content, "A hozzáadott táblázathoz hozzáadott elem 1. sor 2. oszlopának tartalma nem jó");
    assertEquals("Test item 3", lastRow[2].content, "A hozzáadott táblázathoz hozzáadott elem 1. sor 3. oszlopának tartalma nem jó");
    for(let i=0; i< lastRow.length; i++){
        assertEquals(1, lastRow[i].colspan, `A hozzáadott táblázathoz hozzáadott elem 1. sor ${i+1} oszlopának colspan értéke nem jó`);
        assertEquals(1, lastRow[i].rowspan, `A hozzáadott táblázathoz hozzáadott elem 1. sor ${i+1} oszlopának colspan értéke nem jó`);
    }
    return true;
})
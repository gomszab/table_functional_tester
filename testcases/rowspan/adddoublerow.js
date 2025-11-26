test(async () => {
	const form = queryFormById(rowform)
	assertNotUndefined(form)


	const [_table, originalRows ]= queryTableByTbodyId(rowtablebody, true);
    

	setInputValueByid(form, 'elso','Test item 1')
	setInputValueByid(form, 'masodik','Test item 2')
	setInputValueByid(form, 'harmadik','Test item 3')
    setInputValueByid(form, 'negyedik','Test item 4')
    setInputValueByid(form, 'otodik','Test item 5')



	await triggerSubmit(form)
	
	const [_, newRows ]= queryTableByTbodyId(rowtablebody, true);
	assertEquals(originalRows+2, newRows, "Nem fűzött hozzá sort a táblázathoz");
    const lastRow = getLastTwoRowFromTableByTbodyId(rowtablebody);
    assertEquals("Test item 1", lastRow[0].content, "A hozzáadott táblázathoz hozzáadott elem 1. sor 1. oszlopának tartalma nem jó");
    assertEquals("Test item 2", lastRow[1].content, "A hozzáadott táblázathoz hozzáadott elem 1. sor 2. oszlopának tartalma nem jó");
    assertEquals("Test item 3", lastRow[2].content, "A hozzáadott táblázathoz hozzáadott elem 1. sor 3. oszlopának tartalma nem jó");
    assertEquals("Test item 4", lastRow[3].content, "A hozzáadott táblázathoz hozzáadott elem 2. sor 2. oszlopának tartalma nem jó");
    assertEquals("Test item 5", lastRow[4].content, "A hozzáadott táblázathoz hozzáadott elem 2. sor 3. oszlopának tartalma nem jó");
    assertEquals(1, lastRow[0].colspan, "A hozzáadott táblázathoz hozzáadott elem 1. sor 1. oszlopának colspan értéke nem jó");
    assertEquals(2, lastRow[0].rowspan, "A hozzáadott táblázathoz hozzáadott elem 1. sor 1. oszlopának rowspan értéke nem jó");
    for(let i=1; i< lastRow.length; i++){
        assertEquals(1, lastRow[i].colspan, `A hozzáadott táblázathoz hozzáadott elem ${i+1}. cellájának colspan értéke nem jó`);
        assertEquals(1, lastRow[i].rowspan, `A hozzáadott táblázathoz hozzáadott elem ${i+1}. cellájának rowspan értéke nem jó`);
    }
    return true;
})
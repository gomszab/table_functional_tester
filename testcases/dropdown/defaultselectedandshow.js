test(async () => {
    const select = queryFromBody('select[id="tableselector"]')
    assertNotUndefined(select, "nincs select elem a megadott id-val");
    assertNotEmptyString(select.value, "Alapértelmezetten nincs kiválasztva semmi");
    const divList = queryAllFromBody('body > div');
    for (const div of divList) {
        if (div.id === select.value) {
            assertEquals(false, div.classList.contains('hide'), `A ${div.id} azonosítójú elem nincs megjelenítve`);
        } else {
            assertEquals(true, div.classList.contains('hide'), `A ${div.id} azonosítójú elem nincs elrejtve`);
        }

    }
    return true;
}, hascheckbox || !defaultSelected)
test(async () => {
    const select = queryFromBody('select[id="tableselector"]')
    assertNotUndefined(select, "nincs select elem a megadott id-val");
    assertEmptyString(select.value, "Alapértelmezetten ki van választva valami");
    const divList = queryAllFromBody('body > div');
    for (const div of divList) {
        assertEquals(true, div.classList.contains('hide'), `A ${div.id} azonosítójú elem nincs elrejtve`);
    }
    return true;
}, hascheckbox || !defaultEmpty)
 test(async ()=> {

    const form = queryFormById(colform)
	assertNotUndefined(form)

    await triggerSubmit(form)

    return assertEquals(true, window.__submitPrevented, "nincs megakadályozva a form alapértelmezett működésének lefutása");
 })
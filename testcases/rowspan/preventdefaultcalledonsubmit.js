 test(async ()=> {

    const form = queryFormById(rowform)
	assertNotUndefined(form)

    await triggerSubmit(form)

    return assertEquals(true, window.__submitPrevented, "nincs megakadályozva a form alapértelmezett működésének lefutása");
 })
const queryTableById = (id, withRow = false) => {
    const table = document.body.querySelector(`table[id="${id}"]`);
    if(!withRow){
        return [table] ;
    }
	const rowNum = table.querySelectorAll('tbody tr').length;
	return [table, rowNum];
}


const queryTableByTbodyId = (id, withRow = false) => {
	const table = document.body.querySelector(`table:has(tbody[id="${id}"])`);
	if(!table){
		throw new Error(`nincs ilyen ${id}`);
	}
	if(!withRow){
		return [table] ;
	}
	const rowNum = table.querySelectorAll('tbody tr').length;
	return [table, rowNum];
}


const queryFormById = (id) => {
	return document.body.querySelector(`form[id="${id}"]`);
}


const queryInputByIdOfForm = (form, inputId) => {
	return form.querySelector(`input[id="${inputId}"]`);
}


const queryInputByNameOfForm = (form, name) => {
	return form.querySelector(`input[name="${name}"]`);
}


const setInputValueByName = (form, name, value) => {
	queryInputByNameOfForm(form, name).value = value;
}


const setInputValueByid = (form, name, value) => {
	queryInputByIdOfForm(form, name).value = value;
}


const getInputValueByid = (form, name) => {
	return queryInputByIdOfForm(form, name).value;
}


const getInputValueByName = (form, name) => {
	return queryInputByNameOfForm(form, name).value;
}


const queryFromBody = (query) => {
	return document.body.querySelector(query)
}


const queryAllFromBody = (query) => {
	return document.body.querySelectorAll(query)
}

const getErrorFieldContentByInputId = (form, inputId) => {
	return form.querySelector(`input[id="${inputId}"]+.error`).innerText;
}
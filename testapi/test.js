const test = async (callback) => {
	try{
		return await callback();
	}catch(e){
		return e;
	}
}


const promisfySubmitEvent = (form) => {
	return new Promise((resolve) => {
		form.addEventListener('submit', (e) => {
			e.preventDefault();
			resolve();
		})
	})
}

const triggerSubmit = (form) => {
	const promise = promisfySubmitEvent(form);
	form.dispatchEvent(new Event('submit', { bubbles: true, cancelable: true }));
	return promise;
}
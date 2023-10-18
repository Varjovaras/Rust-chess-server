export async function load() {
	const response = await fetch(`http://127.0.0.1:8000/chess`);
	console.log(response);
	const post = await response.json();
	console.log(post);
	return { post };
}

import {json} from '@sveltejs/kit';

export async function POST({request}) {
    const {multi, subtract} = await request.json();
    return json(Math.random() * multi - subtract);
}
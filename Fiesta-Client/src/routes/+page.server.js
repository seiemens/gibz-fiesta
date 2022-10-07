export function load({ params }) {
    return {
        status: 200,
        headers: {
            'access-control-allow-origin': '*'
        },
        body: {
            number: Math.random()
        }
    };

}
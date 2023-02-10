function fetch_count(){
    fetch("http://127.0.0.1:8080/get_count", {method: "GET"})
        .then(res => res.json()
            .then(j => {
                let cnt = document.getElementById("cnt");
                console.log(j.count);
                cnt.innerHTML = `${j.count}`
            }))
}


fetch_count()

sleep = 10

document.getElementById("up").addEventListener("click", ()=>{fetch("http://127.0.0.1:8080/add_count/1", {method: "GET"}); setTimeout(() => fetch_count(), sleep);});
document.getElementById("down").addEventListener("click", ()=>{fetch("http://127.0.0.1:8080/add_count/-1", {method: "GET"}); setTimeout(() => fetch_count(), sleep); });

function ask(){
    var data = JSON.stringify({
        "email" : "email",
        "question" : "42",
    });

    var xhr = new XMLHttpRequest();
    xhr.open("POST", "/question", true);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.send(data);

    // $.ajax({
    //     url: "question",
    //     type: "POST",
    //     data: data,
    //     contentType: "application/json; charset=utf-8",
    //     dataType: "json",
    //     success: function(){
    //     }
    // });
};

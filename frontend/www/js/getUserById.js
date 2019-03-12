function getUserById()
{
  var xhttp = new XMLHttpRequest

  xhttp.onreadystatechange = function() {
    if (this.readyState == 4 && this.status == 200) {
      var oldData = document.getElementById("rtrndata");
      oldData.parentNode.removeChild(oldData);

      var responseObject = JSON.parse(this.responseText);

      var usrRtrn = document.getElementById("userReturn");
      var tbl = document.createElement("table");
      tbl.setAttribute("id", "rtrndata");

      var thd = document.createElement("thead");
      for (var i = 0; i < 4; i++) {
        var th = document.createElement("th");
        switch (i) {
          case 0:
            th.innerHTML = "ID";
            break;
          case 1:
            th.innerHTML = "First Name";
            break;
          case 2:
            th.innerHTML = "Last Name";
            break;
          case 3:
            th.innerHTML = "Banner ID";
            break;
        }
        thd.appendChild(th);
      }
      tbl.appendChild(thd);

      var tbod = document.createElement("tbody")
      var trow = document.createElement("tr");
      for (var j = 0; j < 4; j++) {
        var td = document.createElement("td");
        switch (j) {
          case 0:
            td.innerHTML = responseObject.id;
            break;
          case 1:
            td.innerHTML = responseObject.first_name;
            break;
          case 2:
            td.innerHTML = responseObject.last_name;
            break;
          case 3:
            td.innerHTML = responseObject.banner_id;
            break;
        }
        trow.appendChild(td);
      }
      tbod.appendChild(trow);
      tbl.appendChild(tbod);
      usrRtrn.appendChild(tbl);
    }
  }

  xhttp.open("GET", database+"1", true);
  xhttp.send();
}

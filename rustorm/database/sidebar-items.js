initSidebarItems({"trait":[["Database","Generic Database interface This is the database interface which will should be implemented to you the specifics of each database platform At least all methods on this trait should be implemented for target deployment database A lower level API for manipulating objects in the database"],["DatabaseDDL",""],["DatabaseDev",""]],"struct":[["DbConfig",""],["Pool","This pool contains database that are not necessarily same platform and configs database pools are stored using treemap, with the key is the url of the database owns the database, and only lend out reference when api tries to connect to the database TODO: protect the free form here with locks"]],"enum":[["SqlOption","SqlOption, contains the info about the features and quirks of underlying database"]]});
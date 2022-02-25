print("------- begin lua ---------")

print("CREATING LUA CONTEXT!")

crafter = {
    blocks = {}
}

current_loading_mod = ""

crafter.directory = io.popen"cd":read'*l'

print("LUA CONTEXT CREATED")

print("INITIALIZING DEBUG FUNCTIONS")

crafter.register_block = function(table_data)
    table_data.mod = current_loading_mod
    crafter.blocks[table_data.name] = table_data
end

print("END INITIALIZING DEBUG FUNCTIONS")

print("TESTING DEBUG FUNCTIONS")


print("ENDING TESTING DEBUG FUNCTIONS")


-- this loads all mods into main lua context

print(crafter.directory)

local f = io.popen("dir " .. crafter.directory .. "\\mods /b /ad")

for mod in f:lines() do
    current_loading_mod = mod
    dofile(crafter.directory .. "\\mods\\" .. mod .. "\\main.lua")
end

 print("----- end lua --------- ")
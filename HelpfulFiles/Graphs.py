#!usr/bin/python3

#Useful for parsing files
import re

#EdgeWeightedSymbolDiGraph
class ESDGraph:

    #Init number of Vertices, Edges, and Adjecency lists
    def __init__(self):
        self.V = 0
        self.E = 0
        self.adj = {}

    #Ensure the vertice doesn't exist and add it
    def addVertice(self,V):
        if self.adj.get(V) == None:
            self.adj[V] = {}
            self.V += 1

    #Ensure that the vertices are valid
    #and the edge does not already exist
    #add edge
    def addEdge(self,V1,V2,W=0):
        self.validateVertice(V1)
        self.validateVertice(V2)
        if(self.adj[V1].get(V2) == None):
            self.adj[V1][V2] = W
            self.adj[V2][V1] = W
            self.E += 1

    #Returns the number of adjacent vertices
    def degree(self,V):
        return len(self.adj[V])

    #Determines if the verticy already exists
    #If not an exception is raised
    def validateVertice(self,V):
        if(self.adj.get(V) == None):
            raise Exception(f"{V} is not a valid vertice.")

    #Creates a graph from a file
    #delim is a regex string to split places on
    def GraphFromFile(self, file, delim=",|to|="):
        #Read file
        file = open(file)
        input = file.readlines()
        file.close()

        #prepare delim regex
        delim = re.compile(delim)

        #For each entry add the vertice and edge
        for line in input:
            con = re.split(delim, line.replace(" ", "").strip())
            self.addVertice(con[0])
            self.addVertice(con[1])
            self.addEdge(con[0], con[1], con[2])
            self.addEdge(con[1], con[0], con[2])

    #Prints a tiered list of Vertices, Adjacencies, and Weights
    def __str__(self):
        retstr = f"{self.V} Vertices, {self.E} edges\n"
        for v in self.adj:
            retstr += f"{v}:\n"#\t{self.adj[v]}\n
            for e in self.adj[v]:
                retstr += f"\t{e}\t:\t{self.adj[v][e]}\n"
        return retstr

    #Dinky tests
    def main():
        #Notify User Main is running
        print("running Graph main")
        #Make a Graph!
        G = ESDGraph()
        G.addVertice("a")
        G.addVertice("b")
        G.addVertice("c")
        G.addEdge("a","b")
        G.addEdge("b","c")
        print(G)


#Runs only if run directly
if __name__ == "__main__":
    ESDGraph.main()
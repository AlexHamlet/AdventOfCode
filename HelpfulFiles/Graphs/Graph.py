
class SymbolEdgeWeightedDiGraph:
    def __init__(self):
        #Init number of Vertices, Edges, and Adjecencies
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

    def getAdj(self,V):
        self.validateVertice(V)
        return self.adj.get(V)

    def degree(self,V):
        return len(self.adj[V])

    def validateVertice(self,V):
        if(self.adj.get(V) == None):
            raise Exception(f"{V} is not a valid vertice.")

    def __str__(self):
        retstr = f"{self.V} Vertices, {self.E} edges\n"
        for v in self.adj:
            retstr += f"{v}:\n\t{self.adj[v]}\n"
        return retstr

    def main():
        #Notify User Main is running
        print("running Graph main")
        #Make a Graph!
        G = SymbolEdgeWeightedDiGraph()
        G.addVertice("a")
        G.addVertice("b")
        G.addVertice("c")
        G.addEdge("a","b")
        G.addEdge("b","c")
        print(G)


#Runs only if run directly
if __name__ == "__main__":
    Graph.main()
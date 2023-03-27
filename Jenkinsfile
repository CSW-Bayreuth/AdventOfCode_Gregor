node {
	checkout scm
	
	stage('Installing dependencies') {
		sh 'curl https://sh.rustup.rs/ -sSf | sh'
	}

	def solutions = [
		'15', '16', '17', '18', '19', '20', '21', '22'
	]
	
	def projects = [
		'01', '02', '03', '04', '05', '06', '07', '08', '09', '10',
		'11', '12', '13', '14', '15', '16', '17', '18', '19', '20',
		'21', '22', '23', '24', '25'
	]
	
	for (solution in solutions) {
		for (project in projects) {
			def path = "aoc_${solution}_${project}"
			if (fileExists(path)) {
				stage(path) {
					dir(path) {
						sh 'cargo build'
						
						// sh(script: 'cargo test --logger junit', returnStatus: true)
						// junit(testResults: '**/TestResults.xml', allowEmptyResults: true)
						
						sh 'cargo run'
					}
				}
			}
		}
	}
}
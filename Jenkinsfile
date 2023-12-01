node('linux') {
	checkout scm
	
	stage('Installing dependencies') {
        sh 'curl https://sh.rustup.rs/ -sSf >rustup-init.sh'
        sh 'chmod 777 rustup-init.sh'
        sh './rustup-init.sh -y'
		//sh '$HOME/.cargo/bin/cargo install cargo-test-junit'
    }
    
    def solutions = [
        '15', '16', '17', '18', '19', '20', '21', '22', '23'
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
                dir(path) {
                    stage("${path}: Build") {
                        sh '$HOME/.cargo/bin/cargo build'
                    }
                    //stage("${path}: Test") {
                    //    sh(script: '$HOME/.cargo/bin/cargo test-junit --name TestResults.xml', returnStatus: true)
                    //    junit(testResults: '**/TestResults.xml', allowEmptyResults: true)
                    //}
                    stage("${path}: Run") {
                        sh '$HOME/.cargo/bin/cargo run'
                    }
                }
            }
        }
    }
}
pipeline {
	agent {
		label 'docker && linux'
	}
	environment {
		DOCKER_IMAGE = "rust:1.80"
		DOCKER_ARGS = "-v ${WORKSPACE}/.cargo_registry:/usr/local/cargo/registry"
	}
	options {
		disableConcurrentBuilds()
		disableResume()
	}
	stages {
		stage('Index workspace') {
			steps {
				script {
					def useMultithreading = false

					def years = [
						'15',
						'16',
						'17',
						'18',
						'19',
						'20',
						'21',
						'22',
						'23'
					]

					def days  = [
						'01',
						'02',
						'03',
						'04',
						'05',
						'06',
						'07',
						'08',
						'09',
						'10',
						'11',
						'12',
						'13',
						'14',
						'15',
						'16',
						'17',
						'18',
						'19',
						'20',
						'21',
						'22',
						'23',
						'24',
						'25'
					]

					def branches = [:]

					branches["main"] = {
						stage("AoC Starter") {
							withDockerContainer(image: DOCKER_IMAGE, args: DOCKER_ARGS) {
								catchError(buildResult: 'FAILURE', stageResult: 'FAILURE', catchInterruptions: false) {
									sh 'cargo build --locked'
									sh 'cargo run --locked --bin aoc_starter'
								}
							}
						}
					}

					for (def year in years) {
						for (def day in days) {
							def name = "AoC ${year}-${day}"
							def path = "aoc_${year}_${day}"

							if (fileExists(path)) {
								branches[name] = {
									stage(name) {
										dir(path) {
											withDockerContainer(image: DOCKER_IMAGE, args: DOCKER_ARGS) {
												catchError(buildResult: 'FAILURE', stageResult: 'FAILURE', catchInterruptions: false) {
													sh 'cargo build --locked'

													if (fileExists('tests')) {
														sh 'cargo install cargo2junit'
														sh 'RUSTC_BOOTSTRAP=1 cargo test -- -Z unstable-options --format json --report-time | cargo2junit > results.xml'
														junit(testResults: 'results.xml', allowEmptyResults: true)
													}
												}
											}
										}
									}
								}
							}
						}
					}

					if (useMultithreading) {
						parallel branches
					} else {
						for (def branch in branches.values()) {
							branch()
						}
					}
				}
			}
		}
	}
}
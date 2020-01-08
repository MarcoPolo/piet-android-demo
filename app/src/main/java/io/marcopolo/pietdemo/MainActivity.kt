package io.marcopolo.pietdemo

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.system.Os

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        System.loadLibrary("pietdemo")
        initRust(cacheDir.absolutePath)
        setContentView(PietDemoView(this))
    }

    external fun initRust(cachePath: String)
}
